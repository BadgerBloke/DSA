const getMaxProductOfATriplet = (arr) => {
    if (arr.length < 3) return;

    let firstLargest = Number.MIN_SAFE_INTEGER, secondLargest = Number.MIN_SAFE_INTEGER, thirdLargest = Number.MIN_SAFE_INTEGER;
    let firstSmallest = Number.MAX_SAFE_INTEGER, secondSmallest = Number.MAX_SAFE_INTEGER;

    for (const e of arr) {
        if (e > firstLargest) {
            thirdLargest = secondLargest;
            secondLargest = firstLargest;
            firstLargest = e;
        } else if (e > secondLargest && e < firstLargest) {
            thirdLargest = secondLargest;
            secondLargest = e;
        } else if (e > thirdLargest && e < secondLargest) {
            thirdLargest = e;
        }

        if (e < firstSmallest) {
            secondSmallest = firstSmallest;
            firstSmallest = e;
        } else if (e < secondSmallest) {
            secondSmallest = e;
        }
    }

    const maxProduct = Math.max(firstLargest * secondLargest * thirdLargest, firstSmallest * secondSmallest * firstLargest);
    console.log('Maximum product of a triplet:', maxProduct);
    return maxProduct;
};

const arr = [1, -4, 3, -6, 7, 0];
console.log(getMaxProductOfATriplet(arr));
