import React from 'react'

function DotBackground() {
  return (
    <div className="absolute inset-0 z-[-1]">
        <div className="relative h-full w-full bg-red [&>div]:absolute [&>div]:h-full [&>div]:w-full [&>div]:bg-[radial-gradient(#e5e7eb_1px,transparent_1px)] [&>div]:[background-size:16px_16px] [&>div]:[mask-image:radial-gradient(ellipse_50%_50%_at_50%_50%,#000_70%,transparent_100%)]">
            <div></div> {/* Keep empty div here */}
        </div>
    </div>
  )
}

export default DotBackground