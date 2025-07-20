# Prisma Setup Guide

This guide documents the complete setup process for Prisma with PostgreSQL, including solutions to common issues we encountered.

## Prerequisites

- Node.js (v14 or higher)
- PostgreSQL (v14 or higher)
- npm or yarn package manager
- Homebrew (for macOS)

## Installation

1. Install project dependencies:
   ```bash
   npm install
   # or
   yarn
   ```

2. Install Prisma CLI (if not already installed):
   ```bash
   npm install -D prisma
   # or
   yarn add -D prisma
   ```

## Database Setup

### 1. Start PostgreSQL Service

```bash
# On macOS (using Homebrew)
brew services start postgresql@14
```

### 2. Create Database and User

Connect to PostgreSQL and create a database and user:

```bash
# Connect to PostgreSQL
psql postgres
```

```sql
-- Create database
CREATE DATABASE your_database_name;

-- Create user with password
CREATE USER your_username WITH PASSWORD 'your_secure_password';

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE your_database_name TO your_username;

-- Grant CREATEDB privilege (required for migrations)
ALTER USER your_username CREATEDB;

-- Exit psql
\q
```

## Configuration

### Environment Variables

Create a `.env` file in your project root with:

```env
# Database connection
DATABASE_URL="postgresql://your_username:your_secure_password@localhost:5432/your_database_name?schema=public"

# Optional: Enable debug logging
DEBUG=true
```

### Prisma Schema

Your `prisma/schema.prisma` should include:

```prisma
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

// Define your models here
model User {
  id        String   @id @default(uuid())
  username  String   @unique
  email     String   @unique
  password  String
  apiKey    String   @unique
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
}
```

## Initialize Prisma

1. Generate Prisma Client:
   ```bash
   npx prisma generate
   ```

2. Create and apply initial migration:
   ```bash
   npx prisma migrate dev --name init
   ```

## Development Workflow

### Testing the Connection

We've included a test script at `src/test-db.ts` that verifies the database connection and lists all tables:

```bash
npx ts-node src/test-db.ts
```

### Database Management

Use Prisma Studio to view and edit your database:
```bash
npx prisma studio
```
Then open [http://localhost:5555](http://localhost:5555) in your browser.

### Creating Migrations

When you modify your Prisma schema:
1. Create and apply a new migration:
   ```bash
   npx prisma migrate dev --name descriptive_name
   ```
2. The Prisma Client will be automatically regenerated

## Common Issues & Solutions

### 1. Role Does Not Exist
```
FATAL: role "username" does not exist
```
**Solution**: Create the user in PostgreSQL:
```sql
CREATE USER username WITH PASSWORD 'password' CREATEDB;
```

### 2. Permission Denied to Create Database
```
ERROR: permission denied to create database
```
**Solution**: Grant CREATEDB privilege:
```sql
ALTER USER your_username CREATEDB;
```

### 3. Environment Variables Not Loading
**Solution**: Ensure you're loading `.env` in your entry point:
```typescript
import dotenv from 'dotenv';
dotenv.config();
```

## Production Deployment

For production:
1. Update the `DATABASE_URL` in your environment variables
2. Use environment-specific `.env` files (e.g., `.env.production`)
3. Consider using connection pooling for better performance

## Reset Database

To reset your database (warning: deletes all data):
```bash
npx prisma migrate reset
```

## Additional Resources

- [Prisma Documentation](https://www.prisma.io/docs/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Prisma Studio Guide](https://www.prisma.io/studio)
