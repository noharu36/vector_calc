use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        vecs: [Chars; 2]
    }

    //println!("{:?}", vecs);

    let mut ivec: Vec<String> = Vec::new();
    let mut jvec: Vec<String> = Vec::new();
    let mut kvec: Vec<String> = Vec::new();
    let mut vectors: Vec<Vec<String>> = Vec::new();
    for i in vecs {
        let mut iv = String::new();
        let mut jv = String::new();
        let mut kv = String::new();
        let mut cnt = 0;
        for j in i {
            if j=='+' {
                continue;
            }
            if cnt==0 {
                iv.push(j);
            }
            if cnt==1 {
                jv.push(j);
            }
            if cnt==2 {
                kv.push(j);
            }
            if j=='i' {
                let iv = iv.clone();
                ivec.push(iv);
                cnt += 1;
            }
            if j=='j' {
                let jv = jv.clone();
                jvec.push(jv);
                cnt += 1
            }
            if j=='k' {
                let kv = kv.clone();
                kvec.push(kv);
            }
        }
    }
    vectors.push(ivec);
    vectors.push(jvec);
    vectors.push(kvec);
    //println!("{:?}", vectors);
    let inner_vector = inner_calc(vectors.clone());
    let cross = cross_calc(vectors);
    //println!("{:?}", cross);
    let cross_vector = sum_cross(cross);
    println!("内積 = {}", inner_vector);
    println!("外積 = {}", cross_vector);
}

fn inner_calc(v: Vec<Vec<String>>) -> i32 {
    let mut sum = 0;
    for i in 0..3 {
        let a: &str = &v[i][0];
        let aelement = get_vector(a);
        for j in 0..3 {
            let b:&str = &v[j][1];
            let belement = get_vector(b);
            if aelement.1 != belement.1 {
                continue;
            }
            sum += aelement.0 * belement.0
        }
    }
    return sum
}

fn cross_calc(v: Vec<Vec<String>>) -> Vec<String> {
    let mut sum: Vec<String> = Vec::new();
    for i in 0..3 {
        let a: &str = &v[i][0];
        let aelement = get_vector(a);
        for j in 0..3 {
            let b:&str = &v[j][1];
            let belement = get_vector(b);
            if aelement.1 == belement.1 {
                continue;
            }
            let num = aelement.0 * belement.0;
            let n = num.to_string();
            let mut cross = get_cross(aelement.1, belement.1);

            cross = cross.replace('_', &n);
            if cross.find("--") != None {
                cross = cross.replace("--", "+");
            }
            sum.push(cross);
        }
    }
    return sum
}

fn get_cross(a: char, b: char) -> String {
    if a == 'i' {
        match b {
            'j' => return "+_k".to_string(),
            'k' => return "-_j".to_string(),
            _ => return "err".to_string()
        }
    } else if a == 'j' {
        match b {
            'i' => return "-_k".to_string(),
            'k' => return "+_i".to_string(),
            _   => return "err".to_string()
        }
    } else {
        match b {
            'i' => return "+_j".to_string(),
            'j' => return "-_i".to_string(),
            _   => return "err".to_string()
        }
    }
}

fn sum_cross(v: Vec<String>) -> String {
    let mut isum = 0;
    let mut jsum = 0;
    let mut ksum = 0;
    for i in v {
        let vec: Vec<char> = i.as_str().chars().collect();
        let mut arr: Vec<char> = Vec::new();
        if vec[0] == '+' {
            for i in 1..vec.len() {
                arr.push(vec[i])
            }
        } else {
            arr = vec;
        }
        if arr[0] == '0' {
            continue;
        }
        match arr[arr.len()-1] {
            'i' =>  {
                arr.pop();
                //println!("{:?}", arr);
                let a: String = arr.iter().map(|x| x.to_string()).collect();
                //println!("a = {}",a);
                let n: i32 = a.parse().unwrap();
                isum += n;
            },
            'j' =>  {
                arr.pop();
                //println!("{:?}", arr);
                let a: String = arr.iter().map(|x| x.to_string()).collect();
                //println!("a = {}", a);
                let n: i32 = a.parse().unwrap();
                jsum += n;
            },
            'k' =>  {
                arr.pop();
                //println!("{:?}", arr);
                let a: String = arr.iter().map(|x| x.to_string()).collect();
                //println!("a = {}", a);
                let n: i32 = a.parse().unwrap();
                ksum += n;
            },
            _ => println!("err")
        }
    }
    let mut sum: String = isum.to_string() + "i+" + &jsum.to_string() + "j+" + &ksum.to_string() + "k";

    if sum.find("+-") != None {
        sum = sum.replace("+-", "-")
    }
    return sum;
}

fn get_vector(a: &str) -> (i32, char) {
    let v: Vec<char> = a.chars().collect();
    let c = v[v.len()-1];
    let mut n = String::new();
    for i in 0..v.len()-1 {
        n.push(v[i])
    }
    let num: i32 = n.parse().unwrap();

    return (num, c);
}