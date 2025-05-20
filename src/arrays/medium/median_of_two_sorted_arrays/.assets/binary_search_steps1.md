## Algorithm Steps for Finding the Median of Two Sorted Arrays (Binary Search Method)
#### Using the example arrays:
```
nums1 = [1, 3, 5]
nums2 = [2, 4, 6]
```

## Algorithm Steps:
### 1. Initial Setup:
- Assign ```nums1``` as the smaller array and ```nums2``` as the larger array:
    ```
    nums1 = [1, 3, 5]
    nums2 = [2, 4, 6]
    ```
### 2. Initialize Binary Search Pointers:
    ```
    main_left = 0
    main_right = 3 (length of nums1)
    ```
### 3. First Iteration of Binary Search:
- Calculate ```i``` (partition in ```nums1```):
    ```
    i = (0 + 3) // 2 = 1
    ```
- Calculate ```j``` (partition in ```nums2```):
    ```
    j = half_len - i = 3 - 1 = 2
    ```
- Determine boundary elements:
    ```
    nums1_left  = nums1[i - 1] = nums1[0] = 1
    nums1_right = nums1[i]     = nums1[1] = 3

    nums2_left  = nums2[j - 1] = nums2[1] = 4
    nums2_right = nums2[j]     = nums2[2] = 6
    ```
- Check conditions:
    ```
    nums1_left (1) <= nums2_right (6) → ✅
    nums2_left (4) <= nums1_right (3) → ❌
    ```

Since ```nums2_left > nums1_right```, move the left pointer right:
    ```main_left = i + 1 = 2```

### 4. Second Iteration of Binary Search:
- Calculate new ```i``` (partition in ```nums1```):
    ```
    i = (2 + 3) // 2 = 2
    ```
- Calculate new ```j``` (partition in ```nums2```):
    ```
    j = 3 - 2 = 1
    ```
- Determine boundary elements:
    ```
    nums1_left  = nums1[1] = 3
    nums1_right = nums1[2] = 5

    nums2_left  = nums2[0] = 2
    nums2_right = nums2[1] = 4
    ```
- Check conditions:
    ```
    nums1_left (3) <= nums2_right (4) → ✅
    nums2_left (2) <= nums1_right (5) → ✅
    ```

- Both conditions are met, so we have found the correct partition.

### 5. Calculate the Median:
- Since the combined length (3 + 3) is even, we calculate the median as the average of the two middle values:
    - Left partition max: ```max(nums1_left, nums2_left) = max(3, 2) = 3```
    - Right partition min: ```min(nums1_right, nums2_right) = min(5, 4) = 4```
- Median:
    ```
    (3 + 4) / 2 = 3.5
    ```

![](./binary_search_animation.mp4)