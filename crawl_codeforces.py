import time
import json
import random
import requests
from lxml import etree
from pathlib import Path
from tqdm import tqdm
from collections import defaultdict


def get_suffix(language):
    language = language.lower()
    if "rust" in language:
        return "rs"
    if "c++" in language:
        return "cpp"
    if "py" in language:
        return "py"
    assert False, language


def get_all_submissions(user, max_page=None):
    if max_page is None:
        url = f"https://codeforces.com/submissions/{user}"
        html = requests.get(url).content.decode("utf-8")
        tree = etree.HTML(html)
        max_page = max(map(int, tree.xpath("//span[@pageindex]/@pageindex")))

    submissions = defaultdict(lambda: (0, ''))
    for page_id in tqdm(range(1, max_page + 1)):
        url = f"https://codeforces.com/submissions/{user}/page/{page_id}"
        html = requests.get(url).content.decode("utf-8")
        tree = etree.HTML(html)
        for tr in tree.xpath("//a[@submissionid]/../.."):
            if tr.xpath(".//span[@class='verdict-accepted']"):
                language = get_suffix(tr.xpath(".//td[5]/text()")[0].strip())
                problem_url = tr.xpath("./td[@data-problemid]/a/@href")[0]
                tokens = problem_url.split("/")
                contest_id, problem_id = tokens[-3], tokens[-1].lower()
                submission_id = tr.xpath(".//@data-submission-id")[0]
                url = f"https://codeforces.com/contest/{contest_id}/submission/{submission_id}"
                key = (contest_id, problem_id, language)
                submissions[key] = max(submissions[key], (int(submission_id), url))
    submissions = {(k[0], k[1], k[2], v[1]) for k, v in submissions.items()}
    return submissions


if __name__ == "__main__":
    root_dir = Path("codeforces")
    root_dir.mkdir(parents=True, exist_ok=True)
    submissions_path = root_dir / "submissions.json"

    # Find new submissions
    prev_crawled = set()
    if submissions_path.exists():
        with submissions_path.open() as f:
            prev_crawled = set(tuple(x) for x in json.load(f))
    submissions = get_all_submissions("amoshuangyc", max_page=5)
    new_items = sorted(submissions - prev_crawled, reverse=True)
    print("#submissions:", len(submissions))
    print("#new_items:", len(new_items))

    # Crawl
    for contest_id, problem_id, language, url in tqdm(new_items):
        tqdm.write(f"{contest_id} {problem_id} {language} {url}")

        html = requests.get(url).content.decode("utf-8")
        tree = etree.HTML(html)
        code = tree.xpath('//pre[@id="program-source-text"]/text()')[0]

        path = root_dir / contest_id / f"{problem_id}.{language}"
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("w") as f:
            f.write(code)

        time.sleep(random(3, 7))

    with submissions_path.open("w") as f:
        json.dump(list(submissions), f, indent=1)
