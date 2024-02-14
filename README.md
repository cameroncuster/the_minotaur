# The Minotaur's Birthday Party

To compile & run (from the `birthday_party` directory)
```
cargo run
```

# The Minotaur's Crystal Vase

To compile & run (from the `crystal_vase` directory)
```
cargo run
```

Reference for setting up Rust: [getting started](https://www.rust-lang.org/learn/get-started)

## Potential Solution Strategies

### Option 1

Any guest could stop by and check whether the showroom’s door is open at any time and try to enter the room. While this would allow the guests to roam around the castle and enjoy the party, this strategy may also cause large crowds of eager guests to gather around the door. A particular guest wanting to see the vase would also have no guarantee that she or he will be able to do so and when.

#### Advantages
- simple/easy to implement

#### Disadvantages
- it will be challenging for the scheduler to manage the work every thread is trying to do


### Option 2

The Minotaur’s second strategy allowed the guests to place a sign on the door indicating when the showroom is available. The sign would read “AVAILABLE” or “BUSY.” Every guest is responsible to set the sign to “BUSY” when entering the showroom and back to “AVAILABLE” upon exit. That way guests would not bother trying to go to the showroom if it is not available.

#### Advantages
- easier for the scheduler to handle multiple threads than option 1

#### Disadvantages
- could result in "race" conditions if not implemented carefully

### Option 3

The third strategy would allow the quests to line in a queue. Every guest exiting the room was responsible to notify the guest standing in front of the queue that the showroom is available. Guests were allowed to queue multiple times.

#### Advantages
- efficient algorithm
- mimics the real world solution (in some sense)
- could leverage lock-free queue

#### Disadvantages
- harder to implement than the other options
