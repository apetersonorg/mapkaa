use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    
    io::stdin().read_to_string(&mut buffer)?;   // načtení z STDIN do buffer

    let matrix: Vec<Vec<char>> = buffer         //vytvoření 2D matice charů z dat
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut dig_cnt: usize = 0;
    let mut num_end: usize = 0;
    let mut num_str: usize;
    let mut num_str_iter: usize;
    let mut sum: u32 = 0;
    let num_lines = matrix.len();
    let num_columns = matrix[0].len();
    let num_columns_1 = num_columns-1;
    let mut part: String;
    let mut number: u32;
    let mut check_it: bool=false;
    let mut is_active: bool=false;

    for line in 0..num_lines {
        for chrr in 0..num_columns {                //procházím celou matici
            if matrix[line][chrr].is_digit(10) {    //hledám v ní čísla
                dig_cnt+=1;
                if chrr==num_columns_1 {
                    num_end=chrr+1;
                    check_it=true;                  //našel jsem číslo na konci řádku, musím zkontrolovat zda je to číslo aktivního kontejneru
                }
            } else if dig_cnt!=0 {
                num_end=chrr;
                check_it=true;                      //našel jsem číslo, musím zkontrolovat zda je to číslo aktivního kontejneru
            }

            if check_it {                           //kontroluji zda je kontejner aktivní
                num_str=num_end-dig_cnt;
                num_str_iter=0;
                if num_str>0 {              
                    num_str_iter=num_str-1;
                    if matrix[line][num_str_iter]!='.' {    //kontroluji symbol vlevo před kontejnerem
                        is_active=true;
                    }
                }
                if line>0 {
                    for nmbr in num_str_iter..num_end {
                        if matrix[line-1][nmbr]!='.' {      //kontroluji symboly nad kontejnerem
                            is_active=true;
                        }
                    }
                    if num_end<num_columns_1 {
                        if matrix[line-1][num_end]!='.' {   //kontroluji symbol nad za kontejnerem (vpravo nahoře diagonálně)
                            is_active=true;
                        }
                        if matrix[line][num_end]!='.' {     //kontroluji symbol vpravo za kontejnerem
                            is_active=true;
                        }
                    }
                }

                if line<(num_lines-1) {
                    for nmbr in num_str_iter..num_end {
                        if matrix[line+1][nmbr]!='.' {      //kontroluji symboly pod kontejnerem
                            is_active=true;
                        }
                    }
                    if num_end<num_columns_1 {
                        if matrix[line+1][num_end]!='.' {   //kontroluji symbol pod za kontejnerem (vpravo dole diagonálně)
                            is_active=true;
                        }
                    }
                }

                if is_active {          //když je kontejner aktivní, tak ze znaků udělám řetězec a z něj číslo, které můžu přičíst
                    part = matrix[line][num_str..num_end].iter().collect();
                    number = part.parse().unwrap();
                    sum+=number;
                    is_active=false;
                }
                dig_cnt=0;
                check_it=false;
            }
        }
    }
    println!("{}", sum);
    
    Ok(())
}

