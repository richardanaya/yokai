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
### Time System
- Day/Night Cycle Implementation
  - 24-hour system divided into periods:
    - 明け方 (Dawn, 4:00-6:59)
    - 朝 (Morning, 7:00-11:59)
    - 昼 (Afternoon, 12:00-16:59)
    - 夕方 (Evening, 17:00-19:59)
    - 夜 (Night, 20:00-3:59)
  - Time progression tied to game ticks
  - Configurable day length
  - Time-based event system

### Moon Phase System
- 28-day lunar cycle
- Moon phases affecting gameplay:
  - 新月 (New Moon)
    - Increased yokai activity
    - Reduced visibility
  - 三日月 (Waxing Crescent)
    - Gradual increase in spiritual energy
  - 上弦の月 (First Quarter)
    - Balanced supernatural forces
  - 七日月 (Waxing Gibbous)
    - Growing yokai powers
  - 満月 (Full Moon)
    - Peak of supernatural activity
    - Special events and transformations
  - 十三夜 (Waning Gibbous)
    - Lingering magical effects
  - 下弦の月 (Last Quarter)
    - Weakening of barriers between worlds
  - 二十六夜 (Waning Crescent)
    - Period of transition

### Seasonal System
- Four distinct seasons with unique characteristics:
  - 春 (Spring)
    - Cherry blossom events (花見)
    - Rain frequency increased
    - Specific yokai appearances
    - Growth and renewal themes
  - 夏 (Summer)
    - Festival events (夏祭り)
    - Increased daylight hours
    - Heat effects on gameplay
    - Water-based yokai more active
  - 秋 (Autumn)
    - Foliage changes (紅葉)
    - Harvest moon events
    - Spirit world closer to material world
    - Enhanced yokai transformations
  - 冬 (Winter)
    - Snow accumulation (雪景色)
    - Shorter days
    - Survival challenges
    - Ice/snow yokai prominence

### Weather System
- Dynamic weather patterns:
  - 晴れ (Clear)
    - Standard visibility
    - Normal movement speed
  - 雨 (Rain)
    - Reduced visibility
    - Affects fire-based abilities
    - Empowers water yokai
  - 雪 (Snow)
    - Slowed movement
    - Tracking mechanics
    - Unique combat modifiers
  - 霧 (Fog)
    - Limited visibility range
    - Stealth advantages
    - Enhanced yokai spawning
  - 嵐 (Storm)
    - Combined effects
    - Special events
    - Powerful yokai appearances

### Environmental Interaction System
- Weather affects:
  - Movement speed
  - Combat effectiveness
  - Visibility range
  - Yokai behavior and abilities
- Season influences:
  - Available resources
  - Terrain accessibility
  - Event triggers
  - Background music and sounds
- Time of day impacts:
  - NPC schedules
  - Shop availability
  - Yokai spawn rates
  - Ability effectiveness

### Technical Implementation Details
- Weather transition system
  - Gradual changes between states
  - Particle effects for precipitation
  - Sound effect management
- Season progression
  - Calendar system
  - Event scheduling
  - Terrain modification
- Environmental state machine
  - Complex state transitions
  - Event handling
  - Performance optimization
- Data structures
  - Environmental condition tracking
  - Effect calculation caching
  - State history management

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
