const getThirdLargestElement = (arr) => {
  let firstLargest = -1, secondLargest = -1, thirdLargest = -1;

  for (const e in arr) {
    if (arr[e] > firstLargest) {
      thirdLargest = secondLargest;
      secondLargest = firstLargest;
      firstLargest = arr[e];
    } else if (arr[e] > secondLargest && arr[e] < firstLargest) {
      thirdLargest = secondLargest;
      secondLargest = arr[e];
    } else if (arr[e] > thirdLargest && arr[e] < secondLargest) {
      thirdLargest = arr[e];
    }
  }

  console.log('Third largest:', thirdLargest);
  return thirdLargest;
};

const arr = [12, 35, 1, 10, 34, 1];
console.log(getThirdLargestElement(arr));
