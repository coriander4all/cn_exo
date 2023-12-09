simple rust program to practice mandarin words.
You are prompted a chinese character and must write either the pinyin or the meaning depending on the version.
launch with:
cargo run -- <version>
<version> can be either 1 (write pinyin) or 2 (write meaning)

exit with a sophisticated CTRL+C

when writting pinyin:
	pinyin is written with the tone number after each syllable
when writting meaning:
	write only one of the meanings (NOT IMPLEMENTED YET)

All the words are stored in ./assets/cn.csv
You can add the words respecting the order:
chinese character ; pinyin ; meanings separated by '/' ; comments ; ... (can add other spaces that will be written after checking the answer)

