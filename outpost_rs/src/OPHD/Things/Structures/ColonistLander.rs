#pragma once

#include "Structure.h"

#include "../../Constants.h"
#include "../../Map/Tile.h"

class ColonistLander : public Structure
{

	using Signal = NAS2D::Signal<>;

	ColonistLander(Tile *t) : Structure(constants::ColonistLander,
										"structures/lander_1.sprite",
										StructureClass::Lander,
										StructureID::SID_COLONIST_LANDER),
							  mTile(t)
	{
		maxAge(50);
		turnsToBuild(1);
		repairable(false);
		requiresCHAP(false);
		selfSustained(true);
		integrityDecayRate(2);

		enable();
	}

	Signal::Source &deploySignal() { return mDeploy; }

	void think() override
	{
		if (age() == turnsToBuild())
		{
			mDeploy();
			mTile->index(TerrainType::Dozed);
		}
	}

	Signal mDeploy;

	Tile *mTile;
};
