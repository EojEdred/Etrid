package internal

type Chain struct {
	Blocks []Block
}

func (c *Chain) AddBlock(b Block) {
	c.Blocks = append(c.Blocks, b)
}
