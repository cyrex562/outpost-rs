#pragma once

#include "Structure.h"

#include "../../Constants.h"

class SurfacePolice : public Structure
{

	SurfacePolice() : Structure(constants::SurfacePolice,
								"structures/police_surface.sprite",
								StructureClass::SurfacePolice,
								StructureID::SID_SURFACE_POLICE)
	{
		maxAge(500);
		turnsToBuild(4);

		requiresCHAP(true);
	}

	int getRange() const
	{
		return 5;
	}

	void defineResourceInput() override
	{
		energyRequired(5);
	}
};
