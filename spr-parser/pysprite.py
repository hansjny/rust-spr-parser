import pygame
import numpy as np
import os

WIDTH, HEIGHT = 32, 32  # Sprite size
SPRITES_PER_ROW = 60  # Number of sprites per row
SPACING = 2  # Space between sprites
DIRECTORY = './sprites'  # Directory containing sprite files

# Initialize Pygame
pygame.init()

# Get list of .bin files in the directory
files = [f for f in os.listdir(DIRECTORY) if f.endswith('.bin')]
SPRITE_COUNT = len(files)

# Set up display
ROWS = SPRITE_COUNT // SPRITES_PER_ROW + int(SPRITE_COUNT % SPRITES_PER_ROW > 0)
WINDOW_SIZE = [(WIDTH + SPACING) * SPRITES_PER_ROW, (HEIGHT + SPACING) * ROWS]
screen = pygame.display.set_mode(WINDOW_SIZE)

# Load sprites
sprites = []
for filename in files:
    with open(os.path.join(DIRECTORY, filename), 'rb') as f:
        data = f.read()

    # Convert binary data to numpy array of RGB values
    pixels = np.frombuffer(data, dtype=np.uint8)
    pixels = pixels.reshape((WIDTH, HEIGHT, 3))

    # Create a Pygame Surface from the numpy array
    image = pygame.surfarray.make_surface(pixels)
    sprites.append(image)

# Main loop
running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    # Draw the sprites to the screen
    for i, sprite in enumerate(sprites):
        row = i // SPRITES_PER_ROW
        col = i % SPRITES_PER_ROW
        screen.blit(sprite, (col * (WIDTH + SPACING), row * (HEIGHT + SPACING)))

    # Update the display
    pygame.display.flip()

# Quit Pygame
pygame.quit()