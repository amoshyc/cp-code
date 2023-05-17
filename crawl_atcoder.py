import bs4
import time
import json
from tqdm import tqdm
from pathlib import Path
from random import randint
from urllib.request import urlopen


OUT_DIR = Path("cp-code/atcoder")
OUT_DIR.mkdir(parents=True, exist_ok=True)

# Parse html to get entries
data = {}  # (contest, problem_id, language) -> Dict
with open("index.html", encoding="utf-8") as f:
    rows = bs4.BeautifulSoup(f, "html.parser").select("tbody > tr")
    for row in tqdm(rows):
        tds = row.select("td")
        assert len(tds) == 7

        date = tds[0].select_one("div").get_text().strip()

        problem_url = tds[1].select_one("a").get("href")
        token = problem_url.split("/")[-1].lower()
        index = token.rfind("_")
        contest = token[:index]
        problem_id = token[index + 1 :]

        result = tds[3].select_one("span").get_text().strip()
        if result != "AC":
            continue

        language = tds[4].get_text().strip()
        if "rust" in language.lower():
            suffix = "rs"
        elif "python" in language.lower() or "pypy" in language.lower():
            suffix = "py"
        elif "c++" in language.lower():
            suffix = "cpp"
        else:
            print(language)
            assert False

        source_code_path = OUT_DIR / contest / f"{problem_id}.{suffix}"

        item = {
            "date": date,
            "contest": contest,
            "problem_id": problem_id,
            "problem_url": problem_url,
            "language": language,
            "submission_url": tds[5].select_one("a").get("href"),
            "source_code_path": str(source_code_path.as_posix()),
        }

        key = (item["contest"], item["problem_id"], item["language"])
        if key not in data or date < data[key]["date"]:
            data[key] = item

data = list(data.values())
with open("index.json", "w") as f:
    json.dump(data, f, indent=1)


# Crawl
for item in tqdm(data):
    with urlopen(item["submission_url"], timeout=5) as response:
        html = response.read().decode("utf-8")
    dom = bs4.BeautifulSoup(html, "html.parser")
    code = dom.select_one("#submission-code").get_text()

    path = Path(item["source_code_path"])
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(code)

    time.sleep(randint(2, 4))
