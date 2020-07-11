# 7月11日，Day 15

手敲代码的过程中遇到一个相当难缠的内存问题，似乎难缠的问题都和内存有关系了。

原因是在分配物理页时，采用了自动析构的方式，因此页表中的项，如果不长期持有其引用，会导致其页表失效，造成各种未定义问题，因此如果需要某个页长期有效，就必须一直持有其引用，尤其在做页表映射等和硬件相关的内容时，这个 bug 用了我周六整整一天的时间来搞定。

思考：用一门高级语言来实现 OS 真的是正确的吗。