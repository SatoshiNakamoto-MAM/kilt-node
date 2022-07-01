// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2022 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::{marker::PhantomData, vec::Vec};

use kilt_asset_dids::Asset;
use public_credentials::{Config, Error};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct AssetId<T: Config>(Asset, Option<PhantomData<T>>);

impl<T: Config> TryFrom<Vec<u8>> for AssetId<T> {
	type Error = Error<T>;

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		let asset = Asset::try_from(&value[..]).map_err(|_| Error::<T>::InvalidInput)?;
		Ok(Self(asset, None))
	}
}