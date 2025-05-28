function reverseArrayInGroups(arr, size) {
    let maxGroups = Math.ceil(arr.length / size)

    for (let i=0; i < maxGroups; i++) {
        let groupStart = i * size;
        let groupEnd = i === maxGroups-1 ? arr.length : i * size + size

        let reverseCounter = groupEnd - groupStart -1
        for (let j=0; j<parseInt((groupEnd - groupStart)/2); j++) {
            [arr[j+groupStart],arr[reverseCounter+groupStart]] = [arr[reverseCounter+groupStart],arr[j+groupStart]] 
            reverseCounter--
        }
    }

    console.log("Reversed grouped array:", arr)
}

function reverseInGroups(arr, size) {
    for (let i = 0; i < arr.length; i += size) {
        let left = i;
        let right = Math.min(i + size - 1, arr.length - 1);

        while (left < right) {
            [arr[right], arr[left]] = [arr[left], arr[right]]
            left++
            right--
        }
    }

    console.log("Reversed in gorup:", arr)
}

let arr = [1, 2, 3, 4, 5, 6, 7, 8], size = 10
reverseInGroups(arr, size)
reverseArrayInGroups(arr, size)
