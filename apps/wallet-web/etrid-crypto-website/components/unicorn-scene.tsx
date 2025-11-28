"use client"

import dynamic from 'next/dynamic'
import { useState } from 'react'

// Dynamically import UnicornScene to avoid SSR issues
const UnicornScene = dynamic(
  () => import('unicornstudio-react/next'),
  {
    ssr: false,
    loading: () => (
      <div className="absolute inset-0 bg-gradient-to-br from-purple-900/20 to-blue-900/20 animate-pulse" />
    )
  }
)

interface UnicornSceneWrapperProps {
  projectId: string
  width?: string | number
  height?: string | number
  className?: string
  fps?: number
  scale?: number
  lazyLoad?: boolean
  altText?: string
}

/**
 * Wrapper component for Unicorn Studio 3D scenes
 *
 * Usage:
 * <UnicornSceneWrapper
 *   projectId="YOUR_PROJECT_ID"
 *   className="absolute inset-0 z-0"
 * />
 *
 * To get your project ID:
 * 1. Go to https://unicorn.studio
 * 2. Create your 3D scene
 * 3. Click "Export" > "Embed"
 * 4. Copy the project ID
 */
export default function UnicornSceneWrapper({
  projectId,
  width = "100%",
  height = "100%",
  className = "",
  fps = 60,
  scale = 1,
  lazyLoad = true,
  altText = "3D animated scene"
}: UnicornSceneWrapperProps) {
  const [isLoaded, setIsLoaded] = useState(false)
  const [hasError, setHasError] = useState(false)

  const handleLoad = () => {
    setIsLoaded(true)
    console.log('Unicorn Studio scene loaded successfully')
  }

  const handleError = (error: Error) => {
    setHasError(true)
    console.error('Failed to load Unicorn Studio scene:', error)
  }

  if (hasError) {
    return (
      <div className={`${className} bg-gradient-to-br from-purple-900/10 to-blue-900/10`}>
        {/* Fallback gradient background */}
      </div>
    )
  }

  return (
    <div className={className}>
      <UnicornScene
        projectId={projectId}
        width={width}
        height={height}
        fps={fps}
        scale={scale}
        lazyLoad={lazyLoad}
        altText={altText}
        className="w-full h-full"
        onLoad={handleLoad}
        onError={handleError}
        placeholder={
          <div className="absolute inset-0 bg-gradient-to-br from-purple-900/20 to-blue-900/20 animate-pulse" />
        }
        placeholderClassName="w-full h-full"
        showPlaceholderWhileLoading={true}
        showPlaceholderOnError={true}
      />
    </div>
  )
}
