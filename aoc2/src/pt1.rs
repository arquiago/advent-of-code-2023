use::std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("/home/archago/aoc/aoc2/input.txt").unwrap();
    let mut lc = 0;
    let mut ans = 0;
    let mut flag = true;
    for nline in input.lines(){
        flag = true;
        let mut linen = nline.to_string(); 
        lc = lc + 1;
        println!("game {}", lc);
        let mut hs = HashMap::new();
        hs.insert("blue", 0);
        hs.insert("red", 0);
        hs.insert("green", 0);
        linen = linen.replace("green", "g").replace("red", "r").replace("blue", "b").replace(" ", "")
            .replace(",", "") + ";";
        let ind = linen.find(":").unwrap();

        let mut nline = &linen[ind + 1..];
        println!("{}", nline);
        let mut aux = 0;
        for c in nline.chars(){

            if let Some(digit) = c.to_digit(10) { 
                aux = aux * 10;
                aux = aux + digit;
            }

            else if c == 'g' {
                let auxn = hs.get("green").unwrap();
                hs.insert("green", auxn + aux);
                aux = 0
            }
            else if c == 'r' { 
                println!("aux red eh: {}", aux);
                let auxn = hs.get("red").unwrap();
                hs.insert("red", auxn + aux);
                aux = 0
            }
            else if c == 'b' {
                println!("aux blue eh: {}", aux);
                let auxn = hs.get("blue").unwrap();
                println!("blue eh agora {}", auxn);
                hs.insert("blue", auxn + aux);
                aux = 0;
            }
            else if c == ';' {
                    println!("achou ;");
                    let checkg = *hs.get("green").unwrap();
                    let checkb = *hs.get("blue").unwrap();
                    let checkr = *hs.get("red").unwrap();
                    println!("os valores de g b e r sao {} {} {}", checkg, checkb, checkr);
                    hs.insert("blue", 0);
                    hs.insert("red", 0);
                    hs.insert("green", 0);
                if checkr > 12 || checkg > 13 || checkb > 14{
                   println!("entrou");
                    flag = false;
                    break;
                }
            
            }
        }
        if flag {
            ans+=lc;
        }
    }
    println!("{}", ans );
}
