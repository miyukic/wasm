use wasm_bindgen::prelude::*;

//参考ページ
//https://qiita.com/osanshouo/items/40f087cc79a1446ad7ef

//本来JS側からwasmの関数を呼ぶのは数値型しか扱えない。
//wasm-bindgen が良しなにグルーコードを生成してくれるため,
// String (あるいは Vec<i32> など) を返す Rust 関数を普通に実装すれば JS 側で扱えるようになる。

//Rust側からJavaScriptの関数を呼び出すための宣言
#[wasm_bindgen]
extern "C" {
    //JapaScriptのconsole名前空間を表すアトリビュート
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hoge_func() {

}


//wasmコードを読み込むときに自動的に実行
#[wasm_bindgen(start)]
pub fn run() {
    log("Hello, world");
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
