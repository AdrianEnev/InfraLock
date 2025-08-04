# Development stage
FROM node:18-alpine AS development

WORKDIR /usr/src/app

# Copy package files
COPY package*.json ./
COPY prisma ./prisma/

# Install dependencies
RUN npm install

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Production stage
FROM node:18-alpine AS production

ARG NODE_ENV=production
ENV NODE_ENV=${NODE_ENV}

WORKDIR /usr/src/app

# Copy package files
COPY package*.json ./
COPY prisma ./prisma/

# Install only production dependencies
RUN npm install --only=production

# Copy built application from development stage
COPY --from=development /usr/src/app/dist ./dist

# Expose the application port
EXPOSE 3000

# Start the application
CMD ["npm", "run", "start:prod"]
