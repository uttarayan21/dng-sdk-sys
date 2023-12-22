#ifndef __XMPArtboardPart_Impl_h__
#define __XMPArtboardPart_Impl_h__ 1

// =================================================================================================
// ADOBE SYSTEMS INCORPORATED
// Copyright 2014 Adobe Systems Incorporated
// All Rights Reserved
//
// NOTICE: Adobe permits you to use, modify, and distribute this file in accordance with the terms
// of the Adobe license agreement accompanying it.
// =================================================================================================

#if AdobePrivate
// =================================================================================================
// Change history
// ==============
//
// Writers:
//  SKP Sunil Kishor Pathak
//
// mm-dd-yy who Description of changes, most recent on top
//
// 09-18-14 SKP 1.0 First draft for Asset Relationships API.
//
// =================================================================================================
#endif // AdobePrivate

#include <cstdio>
#include <vector>
#include <string>
#include <cstring>
#include <algorithm>

// Ensure XMP templates are instantiated
#define TXMP_STRING_TYPE std::string 

// Provide access to the API
#include "public/include/XMP.hpp"
#include "XMPAssetManagement/XMPAssetManagementFwdDeclarations.h"
#include "XMPAssetManagement/Interfaces/IXMPPart.h"

namespace AdobeXMPAM { 
	class XMPArtboardPart_Impl : public IXMPArtboardPart {
	public:
		XMPArtboardPart_Impl::XMPArtboardPart_Impl();
		XMPArtboardPart_Impl::XMPArtboardPart_Impl(std::string artboardno, std::string qualifier);

		virtual XMPPartType GetType(void);

		virtual void Set(std::string);
		virtual std::string Get(void);

		virtual void SetDescription(std::string);
		virtual void RemoveDescription(void);
		virtual std::string GetDescription();

		virtual void SetArtboardId(std::string pageNo);
		virtual std::string GetArtboardId(void);

	private:
		std::string artboardid;
		//std::string sQualifier;
		IXMPPart* basePart;
	}; // class XMPTimePart_Impl
}

#endif	// __XMPArtboardPart_Impl_h__
