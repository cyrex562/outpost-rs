// ==================================================================================
// = NAS2D
// = Copyright © 2008 - 2020 New Age Software
// ==================================================================================
// = NAS2D is distributed under the terms of the zlib license. You are free to copy,
// = modify and distribute the software under the terms of the zlib license.
// =
// = Acknowledgment of your use of NAS2D is appreciated but is not required.
// ==================================================================================

#pragma once

#include "Color.h"
#include "../Timer.h"
#include "../signal/signal.rs"

#include <chrono>

namespace NAS2D
{

	class Renderer;


	class Fade
	{

		Fade(Color fadeColor = Color::Black);

		SignalSource<>& fadeComplete();

		void fadeIn(std::chrono::milliseconds fadeTime);
		void fadeOut(std::chrono::milliseconds fadeTime);

		bool isFading() const;
		bool isFaded() const;

		void update();
		void draw(Renderer& renderer) const;


		void setDuration(std::chrono::milliseconds newDuration);

		enum class FadeDirection
		{
			None,
			In,
			Out
		};

		Color mFadeColor{Color::Black};
		FadeDirection mDirection{FadeDirection::None};
		std::chrono::milliseconds mDuration{};
		Timer mFadeTimer;
		Signal<> mFadeComplete;
	};

}
