Feature: Levels can be created in code.
    Scenario: A level contains default tiles when created.
        Given a level's width of 2, height of 2, and depth of 2,
        When the level is created with the desired dimensions,
        Then tile 0, 0, 0 should be a ground tile.
        And tile 0, 1, 0 should be a ground tile.
        And tile 1, 0, 0 should be a ground tile.

    Scenario: A tile can be painted on a level.
        Given a level with a width of 2, height of 2, and a depth of 2,
        When tile 1, 0, 0 is painted as a land tile in the map,
        Then tile 1, 0, 0 should be a blank tile.
