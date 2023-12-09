simple rust program to practice mandarin words.
You are prompted a chinese character and must write either the pinyin or the meaning depending on the version.
launch with:
cargo run -- <version>
<version> can be either 1 (write pinyin) or 2 (write meaning)

exit with a sophisticated CTRL+C

when writting pinyin:
	pinyin is written with the tone number after each syllable
when writting meaning:
	you should write all meanings separated by a '/'
	this is annoying so it will be changed to any meaning
	in the meantime just write a meaning and think of the red as green

All the words are stored in ./assets/cn.csv
You can add the words respecting the order:
chinese character ; pinyin ; meanings separated by '/' ; comments ; ... (can add other spaces that will be written after checking the answer)

