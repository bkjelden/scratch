
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Month {
	Jan,
	Feb,
	Mar,
	Apr,
	May,
	June,
	July,
	Aug,
	Sept,
	Oct,
	Nov,
	Dec,
}

#[derive(Clone)]
#[derive(Debug)]
struct Birthday {
	day: u8,
	month: Month,
} 

fn getDates() -> Vec<Birthday> {
	vec![
		Birthday { day: 15, month: Month::May },
		Birthday { day: 16, month: Month::May },
		Birthday { day: 19, month: Month::May },
		Birthday { day: 17, month: Month::June },
		Birthday { day: 18, month: Month::June },
		Birthday { day: 14, month: Month::July },
		Birthday { day: 16, month: Month::July },
		Birthday { day: 14, month: Month::Aug },
		Birthday { day: 15, month: Month::Aug },
		Birthday { day: 17, month: Month::Aug },
	]
}

fn main() {
	println!("{:?}", getDates());
	println!("{0}", getDates().len());
	let stmt3Dates: Vec<Birthday> = getDates().iter().cloned().filter(|d| statement3(d)).collect();
	println!("{:?}", stmt3Dates);
	println!("{0}", stmt3Dates.len());
	let stmt4Dates: Vec<Birthday> = stmt3Dates.iter().cloned().filter(|d| statement4(d)).collect();
	println!("{:?}", stmt4Dates);
	println!("{0}", stmt4Dates.len());
	let stmt5Dates: Vec<Birthday> = stmt4Dates.iter().cloned().filter(|d| statement5(d)).collect();
	println!("{:?}", stmt5Dates);
	println!("{0}", stmt5Dates.len());
}

fn tellMonth(month: Month, dates: &Vec<Birthday>) -> Vec<Birthday> {
	dates.iter().cloned().filter(|b| b.month == month ).collect()
}

fn tellDay(day: u8, dates: &Vec<Birthday>) -> Vec<Birthday> {
	dates.iter().cloned().filter(|b| b.day == day).collect()
}

fn know(dates: &Vec<Birthday>) -> bool {
	dates.len() == 1
}

fn statement3(date: &Birthday) -> bool {
	let possible_dates = tellMonth(date.month.clone(), &getDates());
	!know(&possible_dates) && possible_dates.iter().all(|d| !know(&tellDay(d.day, &getDates())))
}

fn statement4(date: &Birthday) -> bool {
	let at_first = tellDay(date.day, &getDates());
	!know(&at_first) && know(&at_first.iter().cloned().filter(|b| statement3(&b)).collect())
}

fn statement5(date: &Birthday) -> bool {
	let possible_dates = tellMonth(date.month.clone(), &getDates());
	know(&possible_dates.iter().cloned().filter(|b| statement4(&b)).collect())
}