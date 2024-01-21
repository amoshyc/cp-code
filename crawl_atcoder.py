import time
import json
import requests
from lxml import etree
from pathlib import Path
from tqdm import tqdm


# Retrieve all submissions
user = "amoshuangyc"
submissions = []
start_time = 0
api = "https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user={}&from_second={}"

while True:
    url = api.format(user, start_time)
    time.sleep(3)
    print(url)
    items = requests.get(url).json()
    if len(items) == 0:
        break
    submissions.extend([item for item in items if item["result"] == "AC"])
    max_time = max([item["epoch_second"] for item in items])
    start_time = max(start_time, max_time) + 1

submissions.sort(key=lambda item: item["epoch_second"])

with open("submissions.json", "w") as f:
    json.dump(submissions, f, indent=1)

with open("submissions.json") as f:
    submissions = json.load(f)


# Group the submissions by problem and language
# (problem_id, language) -> url
urls = dict()
for item in submissions:
    language = item["language"].lower()
    if "python" in language or "pypy" in language:
        language = "py"
    elif "rust" in language:
        language = "rs"
    elif "c++" in language:
        language = "cpp"
    else:
        assert False
    url = f"https://atcoder.jp/contests/{item['contest_id']}/submissions/{item['id']}"
    urls[(item["problem_id"], language)] = url


# Crawl
out_dir = Path("atcoder")
out_dir.mkdir(parents=True, exist_ok=True)

for (problem_id, language), url in tqdm(urls.items()):
    index = problem_id.rfind("_")
    contest = problem_id[:index]
    problem = problem_id[(index + 1) :]
    file = out_dir / contest / f"{problem}.{language}"
    file.parent.mkdir(parents=True, exist_ok=True)

    time.sleep(2)
    html = requests.get(url)
    tree = etree.HTML(html.content.decode("utf-8"))
    code = tree.xpath('//pre[@id="submission-code"]//text()')[0]
    code = "\n".join(code.split("\r\n"))

    with open(file, "w", encoding="utf-8") as f:
        f.write(code)
