#pragma once

#include "Structure.h"

#include "../../Constants.h"

class RecreationCenter : public Structure
{

	RecreationCenter() : Structure(constants::RecreationCenter,
								   "structures/recreation_center.sprite",
								   StructureClass::RecreationCenter,
								   StructureID::SID_RECREATION_CENTER)
	{
		maxAge(500);
		turnsToBuild(4);

		requiresCHAP(true);
		hasCrime(true);
	}

	void defineResourceInput() override
	{
		energyRequired(2);
	}
};
