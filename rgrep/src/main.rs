

fn main() {
    let pat = "gets";
    let s = "I have a little shadow that goes in and out with me,
                And what can be the use of him is more than I can see.
                He is very, very like me from the heels up to the head;
                And I see him jump before me, when I jump into my bed.

                The funniest thing about him is the way he likes to grow -
                Not at all like proper children, which is always very slow;
                For he sometimes shoots up taller like an india-rubber ball,
                And he sometimes gets so little that there's none of him at all.";

    for (line_no, line) in s.lines()
    .filter(|s| s.contains(pat))
    .enumerate()
    .map(|(i, s)| (i+1, s))
    // Alternative below, personally like the above better. 
    //
    // .filter_map(move |(i, line)| match line.contains(pat) {
    //     true => Some((i + 1, line)),
    //     false => None,
    // })
    {
        println!("{line_no}: {line}");
    }
}