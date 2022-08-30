# Yokai

A roguelike set in Heian era japan written in Rust using truecolor terminal capabilities and unicode.

<img width="794" alt="Screen Shot 2022-08-28 at 4 45 53 PM" src="https://user-images.githubusercontent.com/294042/187099727-de8cc0d9-f5b2-4e51-97c5-d93f59006785.png">


## Game features

### Time/Weather/Energy lighting model

There's a fairly deep system evolving around time of day, seasons, moons, kami events, energy imbalance, and simulated weather that's evolving with specific effect on ambient lighting.

### Material lighting model

Every object has PBR material, emmision, size, height, and transparency that all effect the flow of light.

### Double wide symbols and color variation

Every entity in the game is represented by a double wide character to make use of unicode symbols and have color alternatives to allow more lively variation of entities.
