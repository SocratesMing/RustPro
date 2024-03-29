|                  | Box                              | Rc (引用计数)                         | RefCell                             |
|------------------|----------------------------------|---------------------------------------|-------------------------------------|
| 所有权            | 独占所有权，只能有一个所有者        | 共享所有权，可以有多个所有者         | 无所有权限制，运行时借用检查          |
| 生态              | 标准库中的一部分                   | 标准库中的一部分                      | std::cell::RefCell                  |
| 运行时开销        | 低                               | 相对较高，引用计数维护开销较大       | 低，但有运行时借用检查的开销         |
| 不可变引用        | 支持                              | 支持                                  | 支持                                |
| 可变引用          | 支持                              | 不支持，必须使用Cell或Mutex获取可变引用 | 支持，运行时检查确保单线程内部的可变引用 |
| 线程安全性        | 需要外部同步机制保证线程安全        | 需要外部同步机制保证线程安全          | 不是线程安全，适用于单线程环境         |
| 运行时借用检查    | 不需要                            | 不需要                               | 需要，用于运行时检查可变引用的规则     |
| 使用场景          | 简单的堆分配，不需要多线程支持      | 共享所有权，不需要多线程支持         | 单线程环境下，运行时可变引用检查较为方便 |
