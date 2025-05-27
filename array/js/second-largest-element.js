const getSecondLargestElement = (arr) => {
  let firstLargest = -1, secondLargest = -1;

  for (const e in arr) {
    if (arr[e] > firstLargest) {
      secondLargest = firstLargest;
      firstLargest = arr[e];
    } else if (arr[e] > secondLargest && arr[e] < firstLargest) {
      secondLargest = arr[e];
    }
  }

  console.log('Second largest:', secondLargest);
  return secondLargest;
};

const arr = [12, 35, 1, 10, 34, 1];
console.log(getSecondLargestElement(arr));
