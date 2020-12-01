def solve():
    nums = sorted([int(x) for x in open('input1.txt', 'r')])

    for i in range(len(nums) - 2):
        l,r = i+1, len(nums) - 1

        while l < r:
            s = nums[i] + nums[l] + nums[r]
            if s > 2020: r -= 1
            elif s < 2020: l += 1
            elif s == 2020: return (nums[i] * nums[l] * nums[r])
                
print(solve())
