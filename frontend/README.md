# Aegis Dashboard

Next.js dashboard for visualizing secret scan events.

## Setup

```bash
# Install dependencies
npm install

# Set environment variables
echo "NEXT_PUBLIC_API_URL=http://localhost:8000" > .env.local

# Run development server
npm run dev
```

Visit http://localhost:3000

## Build for Production

```bash
npm run build
npm start
```

## Environment Variables

- `NEXT_PUBLIC_API_URL` - Backend API URL (default: http://localhost:8000)

## Deployment

Deploy to Vercel:

1. Connect your GitHub repository
2. Set environment variables
3. Deploy automatically on push

