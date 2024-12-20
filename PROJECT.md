# Yokai Roguelike Development Plan

## Core Concept
A roguelike game featuring Japanese yokai (supernatural creatures), using Unicode characters for visualization and incorporating complex environmental systems that affect gameplay.

## Phase 1: Foundation
### Engine Setup
- Initialize Rust project with necessary crates
  - crossterm for terminal manipulation
  - rand for RNG
  - serde for data serialization
  - rodio for audio handling
- Setup error handling and logging system
- Implement game loop with configurable tick rate

### Terminal Rendering
- Unicode character rendering system
  - Support for full-width CJK characters
  - UTF-8 encoding handling
  - Fallback characters for unsupported terminals
- Color support (16/256/RGB depending on terminal)
- Double-buffer rendering for smooth updates
- Screen resize handling

### Player Systems
- Basic player struct with stats
  - HP/MP system
  - Inventory
  - Position tracking
- Movement system
  - 8-directional movement
  - Collision detection
  - Smooth animation transitions

### Map Generation
- Procedural generation system
  - Room and corridor generation
  - Biome-based terrain features
  - Height map for terrain variation
- Map storage and chunking system
- Visibility and FOV calculations
- Basic tile types:
  - Ground (地面) - walkable
  - Wall (壁) - blocking
  - Water (水) - special movement
  - Forest (森) - cover/stealth
  - Mountain (山) - elevation

## Phase 2: Environmental Systems
- Time system (day/night cycle)
- Moon phases system
  - New Moon
  - Waxing Crescent
  - First Quarter
  - Waxing Gibbous
  - Full Moon
  - Waning Gibbous
  - Last Quarter
  - Waning Crescent
- Seasonal system
  - Spring (桜/花見)
  - Summer (夏祭り)
  - Autumn (紅葉)
  - Winter (雪景色)
- Weather conditions
  - Clear (晴れ)
  - Rain (雨)
  - Snow (雪)
  - Fog (霧)
  - Storm (嵐)

## Phase 3: Yokai Implementation
- Basic yokai types:
  - Kappa (河童) - water-dwelling creature
  - Tengu (天狗) - mountain spirit
  - Kitsune (狐) - fox spirit
  - Oni (鬼) - demon
  - Yurei (幽霊) - ghost
- Yokai behaviors tied to environmental conditions
  - Different yokai active during different moon phases
  - Seasonal yokai appearances
  - Weather-specific behaviors

## Phase 4: Dynamic Visuals
- Color system based on:
  - Time of day (朝/昼/夕/夜)
  - Season
  - Weather
  - Moon phase
- Unicode character selection for:
  - Terrain features (山川草木)
  - Items (宝具)
  - Creatures (妖怪)
  - Effects (術法)

## Phase 5: Advanced Features
- Yokai interaction system
  - Combat
  - Dialog
  - Alliances
- Item system with Japanese mythological items
- Special abilities based on time/weather conditions
- Sound effects and background music matching environment

## Phase 6: Polish
- Balance gameplay
- Add story elements
- Implement save/load system
- Add tutorials and help system
- UI improvements and accessibility features

## Technical Considerations
- Unicode handling for proper character display
- Color mixing system for environmental effects
- Performance optimization for weather/time calculations
- Data structure for complex creature behaviors
