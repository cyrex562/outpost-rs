#pragma once

#include "OreRefining.h"

#include "../../Constants.h"

class SeedSmelter : public OreRefining
{
	const int StorageCapacity = 500;

	SeedSmelter() : OreRefining(constants::SeedSmelter,
								"structures/seed_1.sprite",
								StructureClass::Smelter,
								StructureID::SID_SEED_SMELTER)
	{
		maxAge(150);
		turnsToBuild(6);
		requiresCHAP(false);

		storageCapacity(StorageCapacity);
	}

	void defineResourceInput() override
	{
		energyRequired(5);
	}
};
