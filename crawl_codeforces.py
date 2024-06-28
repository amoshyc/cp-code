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


# def get_all_submissions(user, max_page=None):
#     if max_page is None:
#         url = f"https://codeforces.com/submissions/{user}"
#         html = requests.get(url).content.decode("utf-8")
#         tree = etree.HTML(html)
#         max_page = max(map(int, tree.xpath("//span[@pageindex]/@pageindex")))

#     submissions = defaultdict(lambda: (0, ''))
#     for page_id in tqdm(range(1, max_page + 1)):
#         url = f"https://codeforces.com/submissions/{user}/page/{page_id}"
#         html = requests.get(url).content.decode("utf-8")
#         tree = etree.HTML(html)
#         trs = list(tree.xpath('//span[@submissionverdict="OK"]/../..'))
#         if len(trs) == 0:
#             tqdm.write(f'{url} failed')
#             continue
#         else:
#             tqdm.write(f'{url} has {len(trs)} submissions.')

#         for tr in trs:
#             problem_url = tr.xpath("./td[@data-problemid]/a/@href")[0]
#             if 'gym' in problem_url:
#                 continue
#             tokens = problem_url.split("/")
#             contest_id, problem_id = tokens[-3], tokens[-1].lower()
#             language = get_suffix(tr.xpath(".//td[5]/text()")[0].strip())
#             submission_id = tr.xpath(".//@data-submission-id")[0]
#             url = f"https://codeforces.com/contest/{contest_id}/submission/{submission_id}"
#             key = (contest_id, problem_id, language)
#             submissions[key] = max(submissions[key], (int(submission_id), url))
#         time.sleep(1)

#     submissions = {(k[0], k[1], k[2], v[1]) for k, v in submissions.items()}
#     return submissions


def get_all_submissions(user):
    submissions = defaultdict(lambda: (0, ''))
    
    ret = requests.get(f'https://codeforces.com/api/user.status?handle={user}&from=1').json()
    
    for submission in ret['result']:
        if submission['verdict'] != 'OK':
            continue
        contest_id = submission['problem']['contestId']
        if contest_id >= 100_000: # gym
            continue
        submission_id = submission['id']
        language = get_suffix(submission['programmingLanguage'])
        url = f"https://codeforces.com/contest/{contest_id}/submission/{submission_id}"
        key = (str(contest_id), submission['problem']['index'].lower(), language)
        submissions[key] = max(submissions[key], (int(submission_id), url))
    return {(k[0], k[1], k[2], v[1]) for k, v in submissions.items()}

if __name__ == "__main__":
    root_dir = Path("codeforces")
    root_dir.mkdir(parents=True, exist_ok=True)
    submissions_path = root_dir / "submissions.json"

    # Find new submissions
    completed = set()
    if submissions_path.exists():
        with submissions_path.open() as f:
            completed = set(tuple(x) for x in json.load(f))
    submissions = get_all_submissions("amoshuangyc")
    
    new_items = sorted(submissions - completed, reverse=True)
    print("#submissions:", len(submissions))
    print("#new_items:", len(new_items))

    # Crawl
    cnt_failed = 0
    for contest_id, problem_id, language, url in tqdm(new_items):
        html = requests.get(url).content.decode("utf-8")
        tree = etree.HTML(html)
        try:
            code = tree.xpath('//pre[@id="program-source-text"]/text()')[0]
            tqdm.write(f'{contest_id} {problem_id} {language} {url} ok.')
        except:
            tqdm.write(f'{contest_id} {problem_id} {language} {url} failed.')
            cnt_failed += 1
            if cnt_failed >= 5:
                break
            else:
                continue

        path = root_dir / contest_id / f"{problem_id}.{language}"
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("w") as f:
            f.write(code)
        
        completed.add((contest_id, problem_id, language, url))
        time.sleep(random.randint(1, 2))

    with submissions_path.open("w") as f:
        json.dump(list(completed), f, indent=1)
