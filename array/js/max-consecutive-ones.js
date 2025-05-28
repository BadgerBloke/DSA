function getMaxConsecutiveOnesCount(arr) {
    let count = 1
    let maxCount = 1

    for (let i = 1; i < arr.length; i++) {
        if (arr[i] === arr[i-1]) {
            count++
            maxCount = Math.max(maxCount, count)
        } else {
            count = 1
        }
    }

    console.log("Max consecutive ones count:", maxCount)
    return maxCount
}

// let arr = [1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1]
// let arr = [0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]
let arr = [0, 0, 0, 0]
getMaxConsecutiveOnesCount(arr)
