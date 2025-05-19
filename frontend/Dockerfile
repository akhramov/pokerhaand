# Stage 1: Build the application
FROM node:18-alpine AS builder

WORKDIR /app

# Copy package files first for better caching
COPY package.json package-lock.json ./
RUN npm ci

# Copy all other files
COPY . .

# Build the application
RUN npm run build

# Stage 2: Serve the application
FROM node:18-alpine as runner
WORKDIR /app

# Copy from builder
COPY --from=builder /app/build /app/build
COPY --from=builder /app/package.json /app/package-lock.json ./

# Install production dependencies
RUN npm ci --omit=dev

# Environment variables
ENV NODE_ENV=production
ENV PORT=5000

# Expose port
EXPOSE 5000

# Start the app
CMD ["node", "build"]