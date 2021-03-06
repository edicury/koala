## Koala

## What is the library?
Python's pandas implemented for fast, type safe programming in Rust.

## Available Functions & Attributes

- CSV
    ### .read_csv(path: &str) -> CSV
    returns CSV struct reading file from given path
    ```rust
          let mut content = String::new();
          let csv : CSV = dataframe::df::read_csv("test.csv", &mut content); // CSV { headers, values, matrix }
    ```

    ### .get_df() -> DataFrame
    returns DataFrame from a CSV struct
    ```rust
           let mut df = csv.get_df(); // DataFrame { columns, dataset, values }
    ```

- DataFrame
    ### .columns -> Vec<&str>
    returns array of strings, containing column names
    ```rust
           df.columns; // ["name","age"]
    ```
  
     ### .dataset -> Vec<Vec<&str>>
     returns dataset matrix
     ```rust
            df.dataset; // [["bob","30"]
                           ["richard", "25"]]
     ```

     ### .values -> Vec<Pair(&str, Vec<&str>)>
     returns vector of pairs, containing column name, and all column values
     ```rust
            df.values; // [("name", ["bob", "richard"]), ("age", ["30", "25])]
     ```

     ### .max(column: &str) -> f64
     returns max from all values inside a column
     ```rust
            df.max("age"); // 30 as f64
     ```

     ### .max(column: &str) -> f64
     return min from all values inside a column
     ```rust
            df.min("age"); // 25 as f64
     ```

     ### .mean(column: &str) -> f64
     return mean from all values inside a column
     ```rust
            df.mean("age"); // 27.5 as f64
     ```
  
    ### .sum(column: &str) -> f64
    returns sum of all non N/A values from column
    ```rust
           df.sum("age"); // 55 as f64
    ```

     ### [&str] -> Vec<&str>
     string index for DataFrame, returns all values from a given column
     ```rust
            df["age"]; // ["30", "25"]
     ```
     
     ### [usize] -> Vec<&str>
     usize index for DataFrame, returns given row with all columns
     ```rust
            df[0]; // ["bob", "30"]
     ```
  
     ### .iloc(Vec<Range<usize>, Range<usize>>) -> Vec<Vec<&str>>
     returns sliced dataset matrix from given range
     ```rust
            df.iloc([0..2, 0..1].to_vec()); // [["richard"], ["bob"]]
     ```
  
     ### .is_na_col(column: &str) -> bool
     returns if given column on DataFrame has a missing value
     ```rust
            df.is_na_col("age"); // false
     ```
     
     ### .is_na() -> bool
     returns matrix containing missing value bool for each value
     ```rust
            df.is_na(); // [[false, false], [false, false]]
     ```
     
     ### .push(value: Vec<&str>)
     returns matrix containing missing value bool for each value
     ```rust
            df.push(["ann", "20"]);
            df.dataset; // [["richard", "30"], ["bob", "25"], ["ann", "20"]]
     ```
          
     ### .pop(value: Vec<&str>) -> Vec<&str>
     returns matrix containing missing value bool for each value
     ```rust
           df.pop(); // ["ann", "20"]
           df.pop(); // ["bob", "25"]
           df.dataset; // [["richard", "30"]]
     ```
  
     ### .n_uniques(column: &str) -> usize
     returns matrix containing missing value bool for each value
     ```rust
          df.n_uniques("age"); // 2 as usize
     ```
    
     ### .uniques(column: &str) -> Vec<&str>
     returns matrix containing missing value bool for each value
     ```rust
         df.uniques("age"); // ["30", "25"]
     ```
    
     ### .apply(column: &str, function: for<'r> fn(&'r str) -> &'a str)
     applies closure function to each value on given column
     ```rust
         fn in_my_twenties<'r>(age: &str) -> &'r str { "20" }   
         df.apply("age", in_my_twenties);
         df.dataset; // [["richard", "20"], ["bob", "20"]]
     ```
    
    ### .fillna(column: &str, value: &str)
    assigns given value to each N/A value on column
    ```rust
       df.fillna("age", df.mean("age")); // [["richard", "26"], ["bob", "26"]] given bob had no prior age
    ```
  
    ### .dtypes -> HashMap<&str, &str>
    returns type of each column
    ```rust
      df.dtypes // {"age": "numeric", "name": "str" }
    ```