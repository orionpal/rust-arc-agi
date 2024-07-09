# Purpose
Have a general rust framework which other people can use to implement their own solutions to arc-agi prize.

# Running
The default player just makes a random solution
```cargo run```
- This will run the defined players solving function on a random test from training
```cargo run full```
- This will run the solving function on all training and all evaluation problems and then print the #correct/total#
```cargo run some_file_name```
- You can use this to run on a specific test.

# Personal Solution

I want to try at this task using a combination of my ideas for recursive feature identification, emergence, and meta-reasoning framework

The model will have 3 parts:
1. Ground Level
   - This is essentially "reality", and can be thought of as a puzzle set
2. Object Level
   - This is responsible for putting together Macro Level rules that can transform grid A -> grid B
3. Meta Level
   - This is responsible for determining Micro Level systems that influence the Macro Level

"Recursive Feature Identification" will be the means in which the model decides what a "thing" is, and then decides what things make up the original thing (Macro -> Micro)


