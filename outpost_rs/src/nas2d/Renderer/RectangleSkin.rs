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


namespace NAS2D
{
	class Image;
	class Renderer;

	template <typename BaseType>
	struct Rectangle;


	class RectangleSkin
	{

		RectangleSkin(const Image& topLeft, const Image& top, const Image& topRight, const Image& left, const Image& center, const Image& right, const Image& bottomLeft, const Image& bottom, const Image& bottomRight);

		void draw(Renderer& renderer, const Rectangle<float>& rect) const;


		const Image& mTopLeft;
		const Image& mTop;
		const Image& mTopRight;
		const Image& mLeft;
		const Image& mCenter;
		const Image& mRight;
		const Image& mBottomLeft;
		const Image& mBottom;
		const Image& mBottomRight;
	};
}
