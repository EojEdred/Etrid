interface ProgressIndicatorProps {
  current: number;
  total: number;
}

export default function ProgressIndicator({ current, total }: ProgressIndicatorProps) {
  const percentage = (current / total) * 100;

  return (
    <div className="w-full max-w-md mx-auto space-y-3">
      {/* Progress Bar */}
      <div className="relative w-full h-2 bg-white/10 rounded-full overflow-hidden">
        <div
          className="absolute inset-y-0 left-0 bg-gradient-to-r from-purple-500 to-blue-500 rounded-full transition-all duration-500 ease-out"
          style={{ width: `${percentage}%` }}
        />
      </div>

      {/* Step Counter */}
      <div className="flex items-center justify-between text-sm">
        <span className="text-gray-400">
          Step {current} of {total}
        </span>
        <span className="text-purple-400 font-semibold">
          {Math.round(percentage)}% Complete
        </span>
      </div>

      {/* Dots Indicator (Alternative style - optional) */}
      <div className="flex justify-center gap-2 pt-2">
        {Array.from({ length: total }).map((_, index) => (
          <div
            key={index}
            className={`h-2 rounded-full transition-all duration-300 ${
              index < current
                ? 'w-2 bg-purple-500'
                : index === current
                ? 'w-8 bg-purple-500'
                : 'w-2 bg-white/20'
            }`}
          />
        ))}
      </div>
    </div>
  );
}
