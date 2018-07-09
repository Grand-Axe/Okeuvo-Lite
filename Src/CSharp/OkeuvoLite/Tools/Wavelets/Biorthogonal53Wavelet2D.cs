// 
// Biorthogonal53Wavelet2D.cs
//  
// Author:
//       Stefan Moebius
// Date:
//       2016-04-17
// 
// Copyright (c) 2016 Stefan Moebius
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

namespace TurboWavelets
{
	/// <summary>
	/// Implements the two dimensional biorthogonal 5/3 wavelet transformation for arbitrary sizes
	/// </summary>
	public class Biorthogonal53Wavelet2D : Wavelet2D
	{
		/// <summary>
		/// The allowed minimum transformation (limitation of the algorithmn implementation)
		/// </supmmary>
		protected const int   AllowedMinSize    = 3;
        /// <summary>
        /// scale factor
        /// </summary>
		protected const float Scale             = 2.0f;
        /// <summary>
        /// inverse scale factor
        /// </summary>
		protected const float InvScale          = 0.5f;
        /// <summary>
        /// factor for the mean of two values
        /// </summary>
		protected const float Mean              = 0.5f;
        /// <summary>
        /// inverse factor for the mean of two values
        /// </summary>
		protected const float InvMean           = 2.0f;
        /// <summary>
        /// fraction of high-pass added to low-pass (smoothing)
        /// </summary>
		protected const float Smooth            = 0.25f;
        /// <summary>
        /// inverse  fraction of high-pass added to low-pass (smoothing)
        /// </summary>
		protected const float InvSmooth         = 4.0f;

		/// <summary>
		/// A fast implementation of a two-dimensional biorthogonal 5/3 wavelet transformation
		/// for arbitary lenghts (works for all sizes, not just power of 2)
		/// using the lifting scheme. The implementation takes advantage of multiple CPU cores.
		/// </summary>
		/// <param name="width">The width of the transformation</param>
		/// <param name="height">The width of the transformation</param>
		public Biorthogonal53Wavelet2D (int width, int height)
            : base(AllowedMinSize, AllowedMinSize, width, height)
		{   
		}

		/// <summary>
		/// Initalizes a two dimensional wavelet transformation
		/// </summary>
		/// <param name="width">The width of the transformation</param>
		/// <param name="height">The width of the transformation</param>
		/// <param name="minSize">Minimum width/height up to the transformation should be applied</param>
		public Biorthogonal53Wavelet2D (int width, int height, int minSize)
            : base(minSize, AllowedMinSize, width, height)
		{
		}

        #pragma warning disable 1591 // do not show compiler warnings of the missing descriptions
		override protected void TransformRow (float[,] src, float[,] dst, int y, int length)
		{
			if (length >= AllowedMinSize) {
				int half = length >> 1;
				//if the length is even then subtract 1 from "half"
				//as there is the same number of low and high-pass values
				//(Note that "numLFValues" is equal to "half+1") 
				//For a odd length there is one additional quency value (so do not subtract 1)
				//"half" is one less than "numLFValues" as we cannot directly
				//calculate the last value in the for-loop (array bounds)
				if ((length & 1) == 0)
					half--;
				int offSrc = 0;
				// starting offset for high-pass values (= number of low-pass values)
				int offdst = half + 1; 
				int numLFValues = offdst;
	
				float lastHF = 0.0f;
				for (int i = 0; i < half; i++) {
					//calculate the high-pass value by
					//subtracting the mean of the immediate neighbors for every second value
					float hf = src [offSrc + 1, y] - (src [offSrc, y] + src [offSrc + 2, y]) * Mean;
					//smoothing the low-pass value, scale by factor 2 
					//(instead of scaling low frequencies by factor sqrt(2) and
					//shrinking high frequencies by factor sqrt(2)
					//and reposition to have all low frequencies on the left side
					dst [i, y] = (src [offSrc, y] + (lastHF + hf) * Smooth) * Scale;
					dst [offdst++, y] = hf;
					lastHF = hf;
					offSrc += 2; 
				} 
				if ((length & 1) == 0) {
					//the secound last value in the array is our last low-pass value
					dst [numLFValues - 1, y] = Scale * src [length - 2, y]; 
					//the last value is a high-pass value
					//however here we just subtract the previos value (so not really a
					//biorthogonal 5/3 transformation)
					//This is done because the 5/3 wavelet cannot be calculated at the boundary
					dst [length - 1, y] = src [length - 1, y] - src [length - 2, y];
				} else {
					dst [numLFValues - 1, y] = Scale * src [length - 1, y]; 
				}
			} else {
				//We cannot perform the biorthogonal 5/3 wavelet transformation
				//for lengths smaller than 3. We could do a simpler transformation...
				//Here however, we just copy the values from the source to the destination array  
				for (int i = 0; i < length; i++)
					dst [i, y] = src [i, y];
			}
			
		}

		override protected void TransformCol (float[,] src, float[,] dst, int x, int length)
		{
			if (length >= AllowedMinSize) {
				int half = length >> 1;
				//if the length is even then subtract 1 from "half"
				//as there is the same number of low and high-pass values
				//(Note that "numLFValues" is equal to "half+1") 
				//For a odd length there is one additional low-pass value (so do not subtract 1)
				//"half" is one less than "numLFValues" as we cannot directly
				//calculate the last value in the for-loop (array bounds)
				if ((length & 1) == 0)
					half--;
				int offSrc = 0;
				// starting offset for high-pass values (= number of low-pass values)
				int offdst = half + 1; 
				int numLFValues = offdst;
	
				float lastHF = 0.0f;
				for (int i = 0; i < half; i++) {
					//calculate the high-pass value by
					//subtracting the mean of the immediate neighbors for every second value
					float hf = src [x, offSrc + 1] - (src [x, offSrc] + src [x, offSrc + 2]) * Mean;
					//smoothing the low-pass value, scale by factor 2 
					//(instead of scaling low frequencies by factor sqrt(2) and
					//shrinking high frequencies by factor sqrt(2)
					//and reposition to have all low frequencies on the left side
					dst [x, i] = (src [x, offSrc] + (lastHF + hf) * Smooth) * Scale;
					dst [x, offdst++] = hf;
					lastHF = hf;
					offSrc += 2; 
				} 
				if ((length & 1) == 0) {
					//the secound last value in the array is our last low-pass value
					dst [x, numLFValues - 1] = src [x, length - 2] * Scale; 
					//the last value is a high-pass value
					//however here we just subtract the previos value (so not really a
					//biorthogonal 5/3 transformation)
					//This is done because the 5/3 wavelet cannot be calculated at the boundary
					dst [x, length - 1] = src [x, length - 1] - src [x, length - 2];
				} else {
					dst [x, numLFValues - 1] = src [x, length - 1] * Scale; 
				}
			} else {
				//We cannot perform the biorthogonal 5/3 wavelet transformation
				//for lengths smaller than 3. We could do a simpler transformation...
				//Here however, we just copy the values from the source to the destination array  
				for (int i = 0; i < length; i++)
					dst [x, i] = src [x, i];
			}
		}

		override protected void InvTransformRow (float[,] src, float[,] dst, int y, int length)
		{
			if (length >= AllowedMinSize) {
				int half = length >> 1;
				//if the length is even then subtract 1 from "half"
				//as there is the same number of low and high-pass values
				//(Note that "numLFValues" is equal to "half+1") 
				//For a odd length there is one additional low-pass value (so do not subtract 1)
				//"half" is one less than "numLFValues" as we cannot directly
				//calculate the last value in the for-loop (array bounds)
				if ((length & 1) == 0)
					half--;
				// number of low-pass values
				int numLFValues = half + 1;
	
				float lastLF = InvScale * src [0, y] - src [numLFValues, y] * Smooth;
				float lastHF = src [numLFValues, y];
				//Calculate the first two values outside the for loop (array bounds)
				dst [0, y] = lastLF;
				dst [1, y] = lastHF + lastLF * InvScale;
				for (int i = 1; i < half; i++) {
					float hf = src [numLFValues + i, y];
					float lf = InvScale * src [i, y];
					//reconstruct the original value by removing the "smoothing" 
					float lfReconst = lf - (hf + lastHF) * Smooth;
					dst [2 * i, y] = lfReconst;
					//add reconstructed low-pass value (left side) and high-pass value
					dst [2 * i + 1, y] = lfReconst * Mean + hf;
					//add other low-pass value (right side)
					//This must be done one iteration later, as the
					//reconstructed values is not known earlier
					dst [2 * i - 1, y] += lfReconst * Mean;
					lastHF = hf;
					lastLF = lfReconst;
				}
	
				if ((length & 1) == 0) {
					//restore the last 3 values outside the for loop
					//adding the missing low-pass value (right side)
					dst [length - 3, y] += src [numLFValues - 1, y] * Mean * InvScale;
					//copy the last low-pass value
					dst [length - 2, y] = src [numLFValues - 1, y] * InvScale;
					//restore the last value by adding last low-pass value
					dst [length - 1, y] = src [length - 1, y] + src [numLFValues - 1, y] * InvScale; 
				} else {
					//restore the last 2 values outside the for loop
					//adding the missing low-pass value (right side)
					dst [length - 2, y] += src [numLFValues - 1, y] * Mean * InvScale;
					//copy the last low-pass value
					dst [length - 1, y] = src [numLFValues - 1, y] * InvScale;
				}
			} else {
				//We cannot perform the biorthogonal 5/3 wavelet transformation
				//for lengths smaller than 3. We could do a simpler transformation...
				//Here however, we just copy the values from the source to the destination array  
				for (int i = 0; i < length; i++)
					dst [i, y] = src [i, y];				
			}
		}

		override protected void InvTransformCol (float[,] src, float[,] dst, int x, int length)
		{
			if (length >= AllowedMinSize) {
				int half = length >> 1;
				//if the length is even then subtract 1 from "half"
				//as there is the same number of low and high-pass values
				//(Note that "numLFValues" is equal to "half+1") 
				//For a odd length there is one additional low-pass value (so do not subtract 1)
				//"half" is one less than "numLFValues" as we cannot directly
				//calculate the last value in the for-loop (array bounds)
				if ((length & 1) == 0)
					half--;
				// number of low-pass values
				int numLFValues = half + 1;
	
				float lastLF = InvScale * src [x, 0] - src [x, numLFValues] * Smooth;
				float lastHF = src [x, numLFValues];
				//Calculate the first two values outside the for loop (array bounds)
				dst [x, 0] = lastLF;
				dst [x, 1] = lastHF + lastLF * InvScale;
				for (int i = 1; i < half; i++) {
					float hf = src [x, numLFValues + i];
					float lf = InvScale * src [x, i];
					//reconstruct the original value by removing the "smoothing" 
					float lfReconst = lf - (hf + lastHF) * Smooth;
					dst [x, 2 * i] = lfReconst;
					//add reconstructed low-pass value (left side) and high-pass value
					dst [x, 2 * i + 1] = lfReconst * Mean + hf;
					//add other low-pass value (right side)
					//This must be done one iteration later, as the
					//reconstructed values is not known earlier
					dst [x, 2 * i - 1] += lfReconst * Mean;
					lastHF = hf;
					lastLF = lfReconst;
				}
	
				if ((length & 1) == 0) {
					//restore the last 3 values outside the for loop
					//adding the missing low-pass value (right side)
					dst [x, length - 3] += src [x, numLFValues - 1] * Mean * InvScale;
					//copy the last low-pass value
					dst [x, length - 2] = src [x, numLFValues - 1] * InvScale;
					//restore the last value by adding last low-pass value
					dst [x, length - 1] = src [x, length - 1] + src [x, numLFValues - 1] * InvScale; 
				} else {
					//restore the last 2 values outside the for loop
					//adding the missing low-pass value (right side)
					dst [x, length - 2] += src [x, numLFValues - 1] * Mean * InvScale;
					//copy the last low-pass value
					dst [x, length - 1] = src [x, numLFValues - 1] * InvScale;
				}
			} else {
				//We cannot perform the biorthogonal 5/3 wavelet transformation
				//for lengths smaller than 3. We could do a simpler transformation...
				//Here however, we just copy the values from the source to the destination array  
				for (int i = 0; i < length; i++)
					dst [x, i] = src [x, i];				
			}
		}
        #pragma warning restore 1591
	}
}
