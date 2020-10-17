let x = 1;
let array = [1,2,3];
let array = push(array,4);

let factorial = fn(x){
    if (x == 0){
        return 1;
    }
    return x*factorial(x-1);
}

while (x != 10) {
    if (x == 5){
        print("Wo hoo we reached ",x);
    }
    let x = x + 1;
    print("hi");
}

print(factorial(4));
print("My array : ", array);
