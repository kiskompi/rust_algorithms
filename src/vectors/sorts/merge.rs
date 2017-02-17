// ez egy placeholder merge file
pub fn divide(vector:Vec<i32>) ->  Vec<Vec<i32>> {
    let mut subvectors: Vec<Vec<i32>> = vec![];

    if vector.len() >= 4 {
        let mut vec1: Vec<i32> = vec![];
        let mut vec2: Vec<i32> = vec![];

        for i in 0..vector.len()/2{
            vec1.push(vector[i]);
        }

        for i in (vector.len()/2+1)..vector.len(){
            vec2.push(vector[i]);
        }

        for i in divide(vec1){
            subvectors.push(i);
        }

        for i in divide(vec2){
            subvectors.push(i);
        }
    }

    return subvectors;
}

pub fn merge(subvectors:Vec< Vec<i32> >) -> Vec<i32> {
    let mut merged:Vec<i32> = vec![];
    let mut count:usize = 0;

    for vec1 in &subvectors {
        if count % 2 == 0 { continue; }
        let vec2: &Vec<i32> = if count > 0 {
            (subvectors.get(count-1).unwrap())
        } else {
            continue;
        };

        for i in vec1{
            if vec2.get(count) != None {
                merged.push(*i);
            } else if i < &vec2[count] {
                merged.push(vec2[count]);
                merged.push(*i);
            } else {
                // if i >= vec2[count]
                merged.push(*i);
                merged.push(vec2[count]);
            }
        }
        count = count + 1;

    }
    return merged;
}
