# **Reflection 1**

### **a. How much data your publisher program will send to the message broker in one run?**
In `main()`, there is 5 times of invocation: `p.publish_event(...)`. So, in one run, publisher sends **5 event/message* to message broker. Every message contains struct:
```
UserCreatedEventMessage {
    user_id: String,
    user_name: String
```
.
According to Module 09 - Software Architecture document, "how many data" as the number of message, the answer is **5 data/messages**. If I want to be more technical for raw Borsh payload: approximately **24 bytes per message**, total **120 bytes**. It don't include AMQP/RabbitMQ overhead.

### **b. The url of: “amqp://guest:guest@localhost:5672” is the same as in the subscriber program, what does it mean?**
Since both publisher and subscriber use the same AMQP URL, it means they are connected to the same RabbitMQ message broker instance. This allows messages published by the publisher to be received by the subscriber through the broker. Publisher sends event to RabbitMQ, and then subscriber takes or receives event from the same RabbitMQ. So, no matter I use publisher or subscriber program. As long as the programs use the same AMQP URL, it connects to the same RabbitMQ message broker instance.

### **RabbitMQ screenshot**

**RabbitMQ screenshot:** ![RabbitMQ screen capture](/assets/images/screenshot-rabbit-mq.png)

### **Proof that system has sent and processed the event

**Screenshot: **

Subscriber: ![Subscriber](/assets/images/the-program-result-of-subscriber-directory.png)

Publisher: ![Subscriber](/assets/images/the-program-result-of-publisher-directory.png)