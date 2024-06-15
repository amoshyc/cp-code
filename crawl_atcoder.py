import time
import json
import requests
from lxml import etree
from pathlib import Path
from tqdm import tqdm
from collections import defaultdict


def get_suffix(language):
    language = language.lower()
    if "py" in language:
        return "py"
    if "rust" in language:
        return "rs"
    if "c++" in language:
        return "cpp"
    assert False, language


def get_all_submissions(user):
    start_time = 0
    api = "https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user={}&from_second={}"

    submissions = defaultdict(lambda: (0, ''))
    while True:
        url = api.format(user, start_time)
        print(url)
        items = requests.get(url).json()
        if len(items) == 0:
            break

        for item in items:
            if item["result"] == "AC":
                language = get_suffix(item["language"])
                contest_id = item["contest_id"]
                problem_id = item["problem_id"][len(contest_id) + 1 :]
                submission_id = item["id"]
                url = f"https://atcoder.jp/contests/{contest_id}/submissions/{submission_id}"
                key = (contest_id, problem_id, language)
                submissions[key] = max(submissions[key], (int(submission_id), url))

        max_time = max([item["epoch_second"] for item in items])
        start_time = max(start_time, max_time) + 1
        time.sleep(3)

    submissions = {(k[0], k[1], k[2], v[1]) for k, v in submissions.items()}
    return submissions


if __name__ == "__main__":
    root_dir = Path("atcoder")
    root_dir.mkdir(parents=True, exist_ok=True)
    submissions_path = root_dir / "submissions.json"

    # Find new submissions
    prev_crawled = set()
    if submissions_path.exists():
        with submissions_path.open() as f:
            prev_crawled = set(tuple(x) for x in json.load(f))
    submissions = get_all_submissions("amoshuangyc")
    new_items = sorted(submissions - prev_crawled, reverse=True)
    print("#submissions:", len(submissions))
    print("#new_items:", len(new_items))

    for contest_id, problem_id, language, url in tqdm(new_items):
        tqdm.write(f"{contest_id} {problem_id} {language} {url}")
        
        html = requests.get(url).content.decode("utf-8")
        code = etree.HTML(html).xpath('//pre[@id="submission-code"]/text()')[0]
        code = "\n".join(code.split("\r\n"))

        file = root_dir / contest_id / f"{problem_id}.{language}"
        file.parent.mkdir(parents=True, exist_ok=True)
        with open(file, "w", encoding="utf-8") as f:
            f.write(code)

        time.sleep(1)

    with submissions_path.open('w') as f:
        json.dump(sorted(submissions), f, indent=1)