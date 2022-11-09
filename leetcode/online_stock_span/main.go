package main

type Entry struct {
	price int
	count int
}

type StockSpanner struct {
	previous []Entry
}

func Constructor() StockSpanner {
	return StockSpanner{
		previous: make([]Entry, 0),
	}
}

func (this *StockSpanner) Next(price int) int {
	count, i := 1, len(this.previous)-1
	for i >= 0 {
		if this.previous[i].price > price {
			break
		}

		count += this.previous[i].count
		i -= 1
	}

	this.previous = append(this.previous[:i+1], Entry{price: price, count: count})
	return count
}
