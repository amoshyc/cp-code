from typing import List


class PolyHasher:
    def __init__(self, n: int, b: int = 31, p: int = 1_000_000_007):
        self.b = b  # base
        self.p = p  # prime
        self.powr = [1 for _ in range(n)]  # b^i
        self.pinv = [1 for _ in range(n)]  # (b^i)^(-1)
        for i in range(1, n):
            self.powr[i] = self.powr[i - 1] * b % p
        self.pinv[-1] = pow(self.powr[-1], p - 2, p)
        for i in range(n - 2, -1, -1):
            self.pinv[i] = self.pinv[i + 1] * b % p

    def hash(self, s: str) -> List[int]:
        """
        Return the prefix sum.
        """
        h = [ord(c) % self.p * self.powr[i] % self.p for i, c in enumerate(s)]
        pref = [h[0]]
        for i in range(1, len(s)):
            pref.append((pref[i - 1] + h[i]) % self.p)
        return pref

    def range(self, pref: List[int], l: int, r: int) -> int:
        """
        Return the hash of s[l:r].
        """
        if l == r:
            return 0
        elif l == 0:
            return pref[r - 1]
        else:
            return (pref[r - 1] - pref[l - 1]) % self.p * self.pinv[l] % self.p


if __name__ == "__main__":
    hasher1 = PolyHasher(1_000_001, 29, 1_000_000_000 + 7)
    hasher2 = PolyHasher(1_000_001, 31, 1_000_000_000 + 9)

    TC = int(input())
    for _ in range(TC):
        a = input()
        b = input()

        n = len(a)
        pref1_a, pref1_b = hasher1.hash(a), hasher1.hash(b)
        pref2_a, pref2_b = hasher2.hash(a), hasher2.hash(b)

        for i in range(n):
            s1, t1 = hasher1.range(pref1_a, 0, i), hasher1.range(pref1_a, i, n)
            s2, t2 = hasher2.range(pref2_a, 0, i), hasher2.range(pref2_a, i, n)
            concat1 = (s1 * hasher1.powr[n - i] % hasher1.p + t1) % hasher1.p
            concat2 = (s2 * hasher2.powr[n - i] % hasher2.p + t2) % hasher2.p
            if (concat1, concat2) == (pref1_b[-1], pref2_b[-1]):
                print(i)
                break
        else:
            print(-1)
