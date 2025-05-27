const arr = [1, 2, 0, 4, 3, 0, 5, 0];

const pushZerosToEnd = (arr) => {
    let count = 0;

    for (const i in arr) {
        if (arr[i] !== 0) {
            [arr[i], arr[count]] = [arr[count], arr[i]]
            count++
        }
    }

    return arr;
}

console.log(pushZerosToEnd(arr))
