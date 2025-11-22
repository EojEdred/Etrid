package internal

type Block struct {
	Height    int
	Timestamp int64
	Hash      string
	PrevHash  string
	Data      string
}
