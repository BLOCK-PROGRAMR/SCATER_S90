fn main(){

    
}

pub fn fibo(n:u32)->u32{
    if(n==0){
        return 0;
    }
    else if(n==1){
        return 1;
    }
    else{
        return fibo(n-1)+fib(n-2);//how its calling
    }
}