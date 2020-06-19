## Koala

## What is the library?
Python's pandas implemented for fast, type safe programming in Rust.

## Available Functions & Attributes

- CSV
    ### .read_csv(path: &str) -> CSV
    ```rust
          let mut content = String::new();
          let csv : CSV = dataframe::df::read_csv("test.csv", &mut content); // CSV { headers, values, matrix }
    ```

    ### .get_df() -> DataFrame
    ```rust
            let df = csv.get_df(); // DataFrame { columns, dataset, values }
    ```

- Dataframe
    ### .columns -> Vec<&str>
    ```rust
            df.columns // ["name","age"]
    ```
  
     ### .dataset -> Vec<Vec<&str>>
     ```rust
             df.dataset // [["bob","30"]
                            ["richard", "25"]]
     ```
     ### .values -> Vec<Pair(&str, Vec<&str>)>
     ```rust
             df.values // [("name", ["bob", "richard"]), ("age", ["30", "25])]
     ```
     ### .max(column: &str) -> f64
     ```rust
             df.max("age") // 30 as f64
     ```
     ### .max(column: &str) -> f64
     ```rust
             df.min("age") // 25 as f64
     ```
     ### .mean(column: &str) -> f64
     ```rust
            df.mean("age") // 27.5 as f64
     ```
    
     ### [&str] -> Vec<&str>
     ```rust
           df["age"] // ["30", "25"]
     ```
     
     ### [&str] -> Vec<&str>
     ```rust
          df[0] // ["bob", "30"]
     ```