#pragma once

#include "OreRefining.h"

#include "../../Constants.h"


class Smelter : public OreRefining
{
	const int StorageCapacity = 800;


	Smelter() : OreRefining(constants::Smelter,
		"structures/smelter.sprite",
		StructureClass::Smelter,
		StructureID::SID_SMELTER)
	{
		maxAge(600);
		turnsToBuild(9);
		requiresCHAP(false);
		hasCrime(true);
		integrityDecayRate(2);

		storageCapacity(StorageCapacity);
	}


	void defineResourceInput() override
	{
		energyRequired(5);
	}
};
